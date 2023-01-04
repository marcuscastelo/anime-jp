from qbittorrent import Client

class RawDownloader:
    def __init__(self) -> None:
        self._client = Client('http://localhost:8080/', verify=False)
        pass

    def download(self, magnet: str, dest: str = '.'):
        self._client.download_from_link(magnet, savepath=dest)
        pass

    def is_downloading(self):
        return len(self._client.torrents(filter='downloading')) > 0