

const { ApiPromise, WsProvider } = require('@polkadot/api');
const { xxhashAsHex } = require('@polkadot/util-crypto');

const WEB_SOCKET = 'ws://localhost:9944';
const sleep = (ms: number | undefined) => new Promise(resolve => setTimeout(resolve, ms));

const connectSubstrate = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    console.log("connection to substrate successful!!!")
    return api;
};


//subscribe event on node templete
const subscribeEvent = async (api: typeof ApiPromise) => {
    const eventName = 'templateModule.SomethingStored';
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (event.section === 'templateModule') { //订阅指定Module的Event(以templateModule为例)
                console.log('Received custom event:',event.method, event.toHuman());
            }
        });
    });
}


const main = async () => {
    const api = await connectSubstrate();
    await subscribeEvent(api);
    await sleep(6000000);
}



main()
    .then(() => {
        console.log("successfully exited");
        process.exit(0);
    })
    .catch(err => {
        console.log('error occur:', err);
        process.exit(1);
    });


