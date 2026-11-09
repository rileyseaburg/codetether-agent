import { form, modelHeader } from "./payload.mjs";

export async function streamGenerate(prompt, cookie, tokens) {
  const endpoint = `https://gemini.google.com${tokens.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`;
  const query = new URLSearchParams({
    bl: tokens.bl, "f.sid": tokens.sid, hl: "en", pageId: "none",
    _reqid: String(Date.now() % 900000 + 100000), rt: "c",
  });
  return fetch(`${endpoint}?${query}`, {
    method: "POST", body: form(prompt, tokens.at), headers: {
      Cookie: cookie, "User-Agent": tokens.agent, "X-Same-Domain": "1",
      Origin: "https://gemini.google.com", Referer: "https://gemini.google.com/app",
      Accept: "*/*", "Accept-Language": "en-US,en;q=0.9", "Cache-Control": "no-cache",
      Pragma: "no-cache", "sec-fetch-dest": "empty", "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-origin", "x-goog-ext-525001261-jspb": modelHeader(),
    },
  });
}
