from torrentp import TorrentDownloader

class RawDownloader:
    def download(self, anime: str, file: str = '.'):
        TorrentDownloader(anime, file).start_download()