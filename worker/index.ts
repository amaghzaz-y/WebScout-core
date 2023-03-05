import { Router } from "itty-router";
import wasm from "../pkg/webscout_bg.wasm";
import { initSync } from "../pkg/webscout";
import { Scout } from "./scout";
let scout = new Scout();
initSync(wasm);
const router = Router();
router.get("/tokenize/:token", async ({ params }) => {
	if (scout.isSet) {
		let token = scout.tokenize(params.token.toLowerCase())
		token = token.length > 0 ? token : params.token;
		return new Response(token);
	}
	await scout.setup();
	let token = scout.tokenize(params.token.toLowerCase())
	token = token.length > 0 ? token : params.token;
	return new Response(token);
})
router.get("/setup", async () => {
	await scout.setup();
	return new Response(String(scout.isSet))
})
router.get("/search/:query", async ({ params }) => {
	if (scout.isSet) {
		let result = scout.search(params.query)
		return new Response(result);
	}
	await scout.setup();
	let result = scout.search(params.query)
	return new Response(result)
})
// router.get("/index/setup", async () => {
// 	let index = await LANG_PACKS.get("index.pack");
// 	let buffer = await index?.arrayBuffer();
// 	if (buffer) {
// 		await KV_LANG.put("index", buffer)
// 		return new Response("done")
// 	}
// 	return new Response("problem occured")
// })
// router.get("/lang/add/:value", async ({ params }) => {
// 	let pack = await LANG_PACKS.get(`packs/${params.value}.pack`);
// 	let buffer = await pack?.arrayBuffer();
// 	const array = new Uint8Array(buffer as Uint8Array);
// 	if (array.length > 0) {
// 		await KV_LANG.put(`${params.value}`, buffer as ArrayBuffer)
// 		return new Response("done")
// 	}
// 	return new Response(`${array.length}`)
// })

router.get('/', () => new Response("WebScout v0.1.0"));
router.all("*", () => {
	return new Response("wrong way", { status: 404 })
})


addEventListener('fetch', event =>
	event.respondWith(router.handle(event.request))
)