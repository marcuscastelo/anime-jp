from raw_downloader import RawDownloader
import requests
import re
import json
import asyncio
import typing_extensions
import os

def a():
    magnetUrl = r"magnet:?xt=urn:btih:d61c485e0d85d448524db9b267bded648c738f26&amp;dn=%F0%9F%8C%B8Bocchi%20%E6%89%B9%E8%A9%95%E5%BA%A7%E8%AB%87%E4%BC%9A%E3%80%88%E3%81%BC%E3%81%A3%E3%81%A1%E3%83%BB%E3%81%96%E3%83%BB%E3%82%8D%E3%81%A3%E3%81%8F%EF%BC%81%E3%80%89%20-%20%202022-12-27%2020_00&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&amp;tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce"
    RawDownloader().download(magnetUrl)

downloader = RawDownloader()

def b():
    PREFIX = 'https://nyaa.si/?f=0&c=1_4&s=seeders&o=desc&q='
    anime = 'Bocchi'

    url = PREFIX + anime
    print(url)

    response = requests.get(url)
    
    REGEX = r"href=\"(\/view\/[^\"]+?)\" title=\"([^\"]+?)\"(?:.|[\n\r ])+?(magnet:[^\"]+)"

    matches = re.findall(REGEX, response.text)

    files = [match[1] for match in matches]
    magnets = [match[2] for match in matches]

    EPISODE_REGEX_POSTFIX = r".+?-\s*(\d+)\s*"
    EPISODE_REGEX = f'{anime}{EPISODE_REGEX_POSTFIX}'

    episodes = []
    for file, magnet in zip(files, magnets):
        episode = re.findall(EPISODE_REGEX, file)
        if len(episode) > 0:
            episodes.append({
                'file': file,
                'episode': episode[0],
                'magnet': magnet
            })

    
    download_coroutines: list[typing_extensions.Coroutine[typing_extensions.Any, typing_extensions.Any, None]] = []
    downloads_per_episode = 3
    episode_downloads = {}
    for episode in episodes:
        ep_num = episode['episode']
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
