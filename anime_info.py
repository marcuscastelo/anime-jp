from dataclasses import dataclass
import re

EPISODE_REGEX_POSTFIX = r".+?-\s*(\d+)\s*"

@dataclass
class EpisodeDownloadInfo:
    anime: str
    remote_file_name: str
    magnet: str
    seeders: int
    tag: str | None = None

    def validate(self):
        # TODO: assert remote_file_name is ok
        pass

    @property
    def episode(self) -> int:
        EPISODE_REGEX = f'{self.anime}{EPISODE_REGEX_POSTFIX}'
        episode = re.findall(EPISODE_REGEX, self.remote_file_name)
        if len(episode) == 0:
            raise Exception(f'Could not find episode number in {self.remote_file_name}')

        return int(episode[0])
    
@dataclass
class EpisodeGroup:
    anime: str
    tag: str | None
    episodes: list['EpisodeDownloadInfo']

    @staticmethod
    def from_episodes(anime: str, tag: str | None, episodes: list['EpisodeDownloadInfo']) -> 'EpisodeGroup':
        episodes = sorted(episodes, key=lambda e: e.episode)
        if tag is not None:
            episodes = [e for e in episodes if e.tag == tag]
        episodes = [e for e in episodes if e.anime == anime]

        return EpisodeGroup(anime, tag, episodes)