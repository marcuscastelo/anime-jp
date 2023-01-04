import requests
import re
from dataclasses import dataclass

from anime_info import EpisodeDownloadInfo

PREFIX = 'https://nyaa.si/?f=0&c=1_4&s=seeders&o=desc&q='
PARSE_REGEX = r"href=\"(\/view\/[^\"]+?)\" title=\"([^\"]+?)\"(?:.|[\n\r ])+?(magnet:[^\"]+)(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center\">(\d+)"

class EpisodeSearch:
    def __init__(self) -> None:
        pass

    def _get(self, anime: str) -> str:
        url = f'{PREFIX}{anime}'
        response = requests.get(url)
        return response.text

    def _parse(self, anime: str, response: str) -> list[EpisodeDownloadInfo]:
        matches = re.findall(PARSE_REGEX, response)
        return [
            EpisodeDownloadInfo(
                anime=anime,
                remote_file_name=match[1],
                magnet=match[2],
                seeders=int(match[3])
            ) for match in matches
        ]

    def search(self, anime: str) -> list[EpisodeDownloadInfo]:
        response = self._get(anime)
        episodes = self._parse(anime, response)
        return episodes
    

