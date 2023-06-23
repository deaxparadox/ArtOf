import "p/myFile.ts"

const worker1 = new Worker(
    new URL(
        "./worker.ts", 
        import.meta.url
    ).href, 
    {
        type: "module",
    }
)

worker1.postMessage("Something");