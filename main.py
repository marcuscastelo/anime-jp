from raw_downloader import RawDownloader
import requests
import re
import json
import asyncio
import typing_extensions
import os

downloader = RawDownloader()

def b():
    PREFIX = 'https://nyaa.si/?f=0&c=1_4&s=seeders&o=desc&q='
    anime = 'Bocchi'

    url = PREFIX + anime
    print(url)

    response = requests.get(url)
    
    REGEX = r"href=\"(\/view\/[^\"]+?)\" title=\"([^\"]+?)\"(?:.|[\n\r ])+?(magnet:[^\"]+)(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center\">(\d+)"

    matches = re.findall(REGEX, response.text)

    files = [match[1] for match in matches]
    magnets = [match[2] for match in matches]
    seeders = [match[3] for match in matches]

    EPISODE_REGEX_POSTFIX = r".+?-\s*(\d+)\s*"
    EPISODE_REGEX = f'{anime}{EPISODE_REGEX_POSTFIX}'

    episodes = []
    for file, magnet, seeders in zip(files, magnets, seeders):
        episode = re.findall(EPISODE_REGEX, file)
        if len(episode) > 0:
            episodes.append({
                'file': file,
                'episode': episode[0],
                'magnet': magnet,
                'seeders': seeders,
            })

    download_coroutines: list[typing_extensions.Coroutine[typing_extensions.Any, typing_extensions.Any, None]] = []
    downloads_per_episode = 3
    episode_downloads = {}
    zero_seeders_by_episode = {}
    for episode in episodes:
        ep_num = episode['episode']
        seeders = episode['seeders']
        
        if seeders == '0':
            if ep_num not in zero_seeders_by_episode:
                zero_seeders_by_episode[ep_num] = []
            zero_seeders_by_episode[ep_num].append(episode)
            continue

        if ep_num not in episode_downloads:
            episode_downloads[ep_num] = 0

        episode_downloads[ep_num] += 1
            
        if episode_downloads[ep_num] >= downloads_per_episode:
            continue

        async def download_with_timeout(episode: dict, anime: str):
            cwd = os.getcwd()
            save_file = f'{cwd}/output/{anime}/{episode["episode"]}'
            async def download(episode: dict, anime: str):
                print(f"Downloading {episode['episode']} from {episode['file']}")
                downloader.download(episode['magnet'], save_file)

            # Wait for download to finish, or timeout after 10 seconds
            try:
                await asyncio.wait_for(download(episode, anime), timeout=10)
            except asyncio.TimeoutError:
                print(f"Timed out downloading {episode['episode']} from {episode['file']}")
                pass
            
        cr = download_with_timeout(episode, anime)
        download_coroutines.append(cr)
            
    # Wait for all download coroutines to finish
    async def download_all():
        await asyncio.gather(*download_coroutines)
        while downloader.is_downloading():
            print("Waiting for downloads to finish...")
            await asyncio.sleep(1)
        print("Finished downloading all episodes")

    asyncio.run(download_all())
if __name__ == "__main__":
    print("Hello World")
    b()

    pass
