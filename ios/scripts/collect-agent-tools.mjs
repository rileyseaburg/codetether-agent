import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

export async function collectTools(id, headers, out) {
  const saved = await fetch(`https://server.codetether.run/api/session/${id}`, { headers });
  if (!saved.ok) throw new Error(`Session evidence HTTP ${saved.status}`);
  const snapshot = await saved.json();
  const parts = snapshot.messages.flatMap(message => message.content);
  const tools = parts.filter(part => part.type === 'tool_call').map(part => ({
    name: part.name, id: part.id,
    output: parts.find(result => result.type === 'tool_result' && result.tool_call_id === part.id)?.content?.slice(0, 1600)
  }));
  writeFileSync(join(out, 'tools.json'), JSON.stringify(tools, null, 2));
  const text = snapshot.messages.filter(message => message.role === 'assistant').flatMap(message => message.content).filter(part => part.type === 'text').map(part => part.text);
  writeFileSync(join(out, 'assistant-text.json'), JSON.stringify(text, null, 2));
}
