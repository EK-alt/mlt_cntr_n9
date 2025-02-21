import App from "./app";

console.log("Hello from ts-frontend: Again and Again!!!");

const app: App = new App();
const dateTime = await app.getCurrentTime();
console.log("dataTime =", dateTime);

const divDateTime = document.getElementsByClassName("date-time")[0];
divDateTime.innerHTML = JSON.stringify(dateTime) +
    "...With Hot Reload by Webpack.DevServer!";
