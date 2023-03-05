import wasm, { webscout_deserialize_tokenizer } from "../pkg/webscout_bg.wasm";
import { initSync } from "../pkg/webscout";
import { Router } from "itty-router";
import { WebScout } from "../pkg/webscout";

const router = Router();
initSync(wasm);
router.get("/tokenize/:token", async ({ params }) => {
	let webscout = new WebScout("en");
	let pack = await KV_LANG.get("en", "arrayBuffer");
	if (pack !== null) {
		const array = new Uint8Array(pack);
		webscout.deserialize_tokenizer(array);
		let token = webscout.tokenize(params.token);
		return new Response(`token: ${token}`, { status: 200 })
	} else {
		return new Response("something wrong", { status: 404 })
	}
})
router.get("/index/setup", async () => {
	let index = await LANG_PACKS.get("index.pack");
	let buffer = await index?.arrayBuffer();
	if (buffer) {
		await KV_LANG.put("index", buffer)
		return new Response("done")
	}
	return new Response("problem occured")
})
router.get("/lang/add/:value", async ({ params }) => {
	let pack = await LANG_PACKS.get(`packs/${params.value}.pack`);
	let buffer = await pack?.arrayBuffer();
	const array = new Uint8Array(buffer as Uint8Array);
	if (array.length > 0) {
		await KV_LANG.put(`${params.value}`, buffer as ArrayBuffer)
		return new Response("done")
	}
	return new Response(`${array.length}`)
})
router.get("/search/:query", async ({ params }) => {
	let webscout = new WebScout("en");
	let index = await KV_LANG.get("index", "arrayBuffer");
	let pack = await KV_LANG.get("en", "arrayBuffer");
	if (index !== null && pack !== null) {
		const idx = new Uint8Array(index);
		const tokenizer = new Uint8Array(pack);
		webscout.deserialize_index(idx);
		webscout.deserialize_tokenizer(tokenizer);
		let result = webscout.search(params.query);
		return new Response(result, { status: 200 })
	} else {
		return new Response("something wrong", { status: 404 })
	}
})
router.get('/', () => new Response("WebScout v0.1.0"));
router.all("*", () => {
	return new Response("wrong way", { status: 404 })
})


addEventListener('fetch', event =>
	event.respondWith(router.handle(event.request))
)