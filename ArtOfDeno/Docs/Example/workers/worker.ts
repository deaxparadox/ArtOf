// self.onmessage = async (e) => {
//     const { filename } = e.data;
//     const text = await Deno.readTextFile(filename);
//     console.log(text);
//     self.close();
//   };

self.onmessage = async (e) => {
    console.log(e.data)
}