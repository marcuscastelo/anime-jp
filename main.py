from raw_downloader import RawDownloader
import requests
import re
import json
import asyncio
import typing_extensions
import os
import enlighten

from anime_search import EpisodeSearch
from anime_info import EpisodeGroup, EPISODE_REGEX_POSTFIX, get_episode_of_filename

downloader = RawDownloader()

def download_raws(anime: str, save_name: str = ...):
    episodeSearch = EpisodeSearch()
    episodes = episodeSearch.search(anime)
    episodes = [e for e in episodes if re.match(EPISODE_REGEX_POSTFIX, e.remote_file_name) ]

    tag = '[Ohys-Raws]'
    for episode in episodes:
        if tag in episode.remote_file_name:
            episode.tag = tag
    
    episode_group = EpisodeGroup.from_episodes(anime, tag, episodes)

    print(f'Found {len(episode_group.episodes)} episodes')
    for episode in episode_group.episodes:
        print(f'Tag: {episode.tag}, Episode: {episode.episode}, Seeders: {episode.seeders}')
        print(f'File: {episode.remote_file_name}')
        print(f'Magnet: {episode.magnet}')
        print()

        cwd = os.getcwd()
        save_name = save_name if save_name is not ... else anime
        save_folder_path = f'{cwd}/output/{save_name}'

        if episode.seeders == '0':
            print(f'No seeders for {episode}')
            continue

        downloader.download(episode.magnet, save_folder_path)

    # Wait for all download coroutines to finish
    async def download_all():
        print("Waiting for downloads to finish...")
        downloader._client.force_start('all', True)
        torrents = downloader._client.torrents(filter='downloading')
        
        manager = enlighten.get_manager()

        pbars = {}
        while downloader.is_downloading():
            await asyncio.sleep(1)
            torrents: list[dict] = downloader._client.torrents(filter='downloading')
             
            name_progress = { 
                torrent['name']: f'{torrent["progress"]*100:.2f}%' 
                for torrent in torrents 
            }
            for name, progress in sorted(name_progress.items(), key=lambda x: x[0], reverse=True):
                if name not in pbars:
                    pbars[name] = manager.status_bar(
                        status_format=u'{demo}{fill}{progress} ({elapsed})', unit='%', color='green', demo=name, progress="0", max_value=100)

                pbars[name].update(progress=progress)

        print("Finished downloading all episodes")

    asyncio.run(download_all())

# Returns the exact name of the anime
def download_subtitles(anime: str) -> str:
    body = requests.get('https://kitsunekko.net/dirlist.php?dir=subtitles%2Fjapanese%2F').text
    # sanitize body to utf-8
    body = body.encode('ascii', 'ignore').decode('ascii')
        
    all_anime = re.findall(r"<tr><td colspan=\"2\"><a href=\"/([^\"]+).+?<strong>([^<]+)", body, re.MULTILINE)
    all_anime = {str(name): str(url) for url, name in all_anime}

    matching = [name for name in all_anime if anime.lower() in name.lower()]

    if not matching:
        print(f'Anime {anime} not found')
        return 'AAAAAAAAAAAAAAAAANOTFOUND'
    elif len(matching) > 1:
        print(f'Found multiple anime for {anime}:')
        for anime in matching:
            print(f' - {anime}')
        return 'AAAAAAAAAAAAAAAAANOTFOUND'
    else:
        print(f'Found anime {anime}')
    
    print(f'Assuming "{anime}" is "{matching[0]}"')
    anime = matching[0]

    url = all_anime[anime]
    url = f'https://kitsunekko.net/{url}'

    body = requests.get(url).text
    all_subs = re.findall(r'<tr><td><a href="([^"]+).+?<strong>([^<]+)', body, re.MULTILINE)
    all_subs = {str(name): str(url) for url, name in all_subs}

    print(f'Found {len(all_subs)} subtitles')
    for name, url in all_subs.items():
        print(f' - {name}')

    print(f'Downloading subtitles for {anime}')
    for name, url in all_subs.items():
        # download subtitle .srt directly
        file_path = f'output/{anime}/subs/{name}'
        if os.path.exists(file_path):
            print(f'Subtitle {name} already exists, skipping')
            continue

        if url.endswith('.srt'):
            print(f'Downloading {name}')
            body = requests.get(f'https://kitsunekko.net/{url}').text
            os.makedirs(f'output/{anime}/subs', exist_ok=True)
            with open(file_path, 'w') as f:
                f.write(body)

    return anime

def rename_all(anime_folder: str):
    subs = [f for f in os.listdir(f'{anime_folder}/subs') ]
    raws = [f for f in os.listdir(f'{anime_folder}') if f != 'subs' ]

    episode_sub_map = {}
    for sub in subs:
        print(f'Getting episode of {sub=}')
        episode = get_episode_of_filename(sub)
        episode_sub_map[episode] = sub
    
    episode_raw_map = {}
    for raw in raws:
        print(f'Getting episode of {raw=}')
        episode = get_episode_of_filename(raw)
        episode_raw_map[episode] = raw

    for episode, sub in episode_sub_map.items():
        if episode not in episode_raw_map:
            print(f'No raw for episode {episode}')
            continue

        raw = episode_raw_map[episode]
        raw_base, raw_ext = os.path.splitext(raw)
        _, sub_ext = os.path.splitext(sub)
        os.rename(f'{anime_folder}/{raw}', f'{anime_folder}/{raw_base}.{raw_ext}')
        os.rename(f'{anime_folder}/subs/{sub}', f'{anime_folder}/subs/{raw_base}.{sub_ext}')

if __name__ == "__main__":
    anime_loose = input('Anime: ') or 'Bocchi'
    anime_from_subs = download_subtitles(anime_loose)
    download_raws(anime_loose, anime_from_subs)
    rename_all(f'output/{anime_from_subs}')
    pass
