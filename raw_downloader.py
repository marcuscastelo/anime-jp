from torrentp import TorrentDownloader

class RawDownloader:
    def download(self, anime: str):
        TorrentDownloader(anime, '.').start_download()