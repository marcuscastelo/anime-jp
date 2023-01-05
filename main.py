from raw_downloader import RawDownloader
import requests
import re
import json
import asyncio
import typing_extensions
import os
import enlighten

from anime_search import EpisodeSearch
from anime_info import EpisodeGroup, EPISODE_REGEX_POSTFIX

downloader = RawDownloader()

def start(anime: str):
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
        save_path = f'{cwd}/output/{anime}/{episode.episode}'

        if episode.seeders == '0':
            print(f'No seeders for {episode}')
            continue

        downloader.download(episode.magnet, save_path)

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
if __name__ == "__main__":
    start(input('Anime: ') or 'Pop Team')

    pass
