import axios from "axios"

export default class App {
    constructor() {

    }

    async getCurrentTime() {
        const props = {
            api: ''
        }
        // http://localhost:9093
        const url = 'https://actix-webapp-n3/actix-webapp/';
        const { isLoading, error, data, isFetching } = await this.useQuery(url);

        return data
    }

    async useQuery(url: string) {
        const res = await fetch(url, {
            method: "GET",
            // headers: {
            //     'Content-Type': "application/json"
            // },
        })

        if (!res.ok) {
            throw new Error(`HTTP error! status: ${res.status}`);
        }

        const data = await res.json();
        console.log("JSON data =", data);

        return {
            isLoading: undefined, error: undefined, data: data, isFetching: undefined
        };
    }

}