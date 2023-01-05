from dataclasses import dataclass
import re

EPISODE_REGEX_POSTFIX = r".+?(?:-\s*()(\d+)|S(\d+)E(\d+))"

def get_episode_of_filename(filename: str, *, prefix: str = '') -> str:
    EPISODE_REGEX = f'{prefix}.*?{EPISODE_REGEX_POSTFIX}'
    episode = re.findall(EPISODE_REGEX, filename)
    if len(episode) == 0:
        raise Exception(f'Could not find episode number in {filename}')
    
    if episode[0][2] != '':
        return f'S{episode[0][2]}E{episode[0][3]}'
    else:
        return f'S01E{episode[0][1]}'

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
    def episode(self) -> str:
        return get_episode_of_filename(self.remote_file_name, prefix = self.anime)
    
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