interface JsonRpcOpts {
    url: string;
    auth: string;
}

interface JsonRpcRes {
    jsonrpc: "2.0";
    id: number | string;
    result?: any;
    error?: { code: number; message: string; data?: any };
}

export class JsonRpcClient {
    private nextID = 1;
    private options: JsonRpcOpts;

    constructor(options: JsonRpcOpts) {
        this.options = options;
    }

    async req(method: string, params: any[]): Promise<JsonRpcRes> {
        const res = await fetch(this.options.url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": 'Basic ' + Buffer.from(this.options.auth).toString('base64')
            },
            body: JSON.stringify({
                jsonrpc: "2.0",
                id: this.nextID++,
                method,
                params
            })
        });

        return (await res.json()) as JsonRpcRes;
    }
} 