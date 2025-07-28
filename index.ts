import { fuzz } from "./src/bindings";

(async () => {
  const results = await fuzz("https://google.com", "./wordlist.txt");
  console.log(results);
})();
