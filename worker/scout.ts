import { WebScout } from "../pkg/webscout";
export class Scout {
	webscout: any;
	isSet: boolean;
	constructor() {
		this.isSet = false;
	}
	async setup() {
		this.webscout = new WebScout("en");
		let idx = await KV_LANG.get("index", "arrayBuffer");
		let en = await KV_LANG.get("en", "arrayBuffer");
		if (idx !== null && en !== null) {
			this.webscout.deserialize_tokenizer(new Uint8Array(en));
			this.webscout.deserialize_index(new Uint8Array(idx));
			this.webscout.setup();
			this.isSet = true;
		}
	}
	tokenize(word: string): string {
		let token = this.webscout.tokenize(word);
		return token;
	}
	search_all(query: string): string {
		let result = this.webscout.search_all(query);
		return result;
	}
	search(query: string): string {
		let result = this.webscout.search_above_average(query);
		return result;
	}
}