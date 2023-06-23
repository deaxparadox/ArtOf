import { Worker } from "worker_threads";

console.log("hello typescript")

let worker1 = new Worker("./main/another.ts");
worker1.postMessage("It's personal")