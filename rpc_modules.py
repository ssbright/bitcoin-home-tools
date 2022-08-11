from bitcoinrpc import BitcoinRPC

async def getaddressinfo(address: str):
    "https://developer.bitcoin.org/reference/rpc/getaddressinfo.html"
    return await BitcoinRPC.acall("getaddressinfo", [address])