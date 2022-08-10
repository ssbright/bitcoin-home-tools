import asyncio
from bitcoinrpc import BitcoinRPC
from dotenv import load_dotenv
import os

load_dotenv()
rpc_user = os.getenv('RPC_USER')
rpc_password = os.getenv('RPC_PASSWORD')
ip = os.getenv('IP')
card=os.getenv('CARD')

rpc = BitcoinRPC(ip, rpc_user, rpc_password)

async def connections():
    async with BitcoinRPC(ip, rpc_user, rpc_password) as rpc:
        print(await rpc.getconnectioncount())



if __name__ == "__main__":
    asyncio.run(connections())