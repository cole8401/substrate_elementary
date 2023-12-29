
import {ApiPromise, WsProvider} from "@polkadot/api";

const WEB_SOCKET_URL = "ws://127.0.0.1:9944";

async function connect() {
    const wsProvider = new WsProvider(WEB_SOCKET_URL);
     // Create our API with a default connection to the local node
    const api = await ApiPromise.create({ provider: wsProvider});
    await api.isReady;
    return api;
}

async function getEvents(api: ApiPromise) {
    api.query.system.events((events: any[])=>{
        events.forEach((record: any)=>{
            const {event} = record;
            if (event.section ==='templateModule' && event.method === 'SomethingStored'){
                console.log('\nevent method is:',event.method);
                console.log('\nthe event data is:',event.data.toString());
                console.log('\nthe updated value is:',event.data[0].toString());
            }
        });
    });
}

async function sleep(ms: number) {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve('');
        }, ms)
    });
}
async function main() {
    const api = await connect();
    getEvents(api);
    await sleep(50000);
}

main().catch((e)=>{
    console.error("Something went wrong", e);
});
