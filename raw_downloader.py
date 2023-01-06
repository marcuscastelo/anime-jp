from qbittorrent import Client

class RawDownloader:
    def __init__(self) -> None:
        try:
            self._client = Client('http://localhost:8080/', verify=False)
        except Exception as e:
            # Print original error message
            print(e)

            # Print custom error message
            print('Could not connect to qBittorrent. Is it running?')
            exit(1)

    def download(self, magnet: str, dest: str = '.'):
        self._client.download_from_link(magnet, savepath=dest)
        pass

    def is_downloading(self):
        return len(self._client.torrents(filter='downloading')) > 0