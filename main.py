from raw_downloader import RawDownloader
import requests
import re
import json
import asyncio
import typing_extensions
import os

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
        while downloader.is_downloading():
            await asyncio.sleep(1)
            print("Still downloading...")
            torrents = downloader._client.torrents(filter='downloading')
             
            a = { torrent['name']: f'{torrent["progress"]*100:.2f}%' for torrent in torrents }

            for name, progress in sorted(a.items(), key=lambda x: x[0], reverse=True):
                print(f'{progress}\t\t{name}')

        print("Finished downloading all episodes")

    asyncio.run(download_all())
if __name__ == "__main__":
    start(input('Anime: '))

    pass
