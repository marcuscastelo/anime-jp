from raw_downloader import RawDownloader

def a():
    magnetUrl = r"magnet:?xt=urn:btih:d61c485e0d85d448524db9b267bded648c738f26&amp;dn=%F0%9F%8C%B8Bocchi%20%E6%89%B9%E8%A9%95%E5%BA%A7%E8%AB%87%E4%BC%9A%E3%80%88%E3%81%BC%E3%81%A3%E3%81%A1%E3%83%BB%E3%81%96%E3%83%BB%E3%82%8D%E3%81%A3%E3%81%8F%EF%BC%81%E3%80%89%20-%20%202022-12-27%2020_00&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&amp;tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce"
    RawDownloader().download(magnetUrl)
    
if __name__ == "__main__":
    print("Hello World")
    a()

    pass
