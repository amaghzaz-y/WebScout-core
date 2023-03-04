import wasm from "../pkg/webscout_bg.wasm";
import init, { initSync } from "../pkg/webscout";
import { Router } from "itty-router";
import { WebScout } from "../pkg/webscout";

const router = Router();
initSync(wasm);
router.get("/tokenize/:token", async ({ params }) => {
	let webscout = new WebScout("en");
	let pack = await LANG_PACKS.get("packs/en.pack");
	let blob = await pack?.blob();
	let buffer = await blob?.arrayBuffer();
	if (buffer !== undefined) {
		const array = new Uint8Array(buffer);
		webscout.deserialize_tokenizer(array);
		let token = webscout.tokenize(params.token);
		return new Response(`token: ${token}`, { status: 200 })
	} else {
		return new Response("something wrong", { status: 404 })
	}
})
router.all("*", () => {
	return new Response("wrong way homie", { status: 404 })
})


addEventListener('fetch', event =>
	event.respondWith(router.handle(event.request))
)