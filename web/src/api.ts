export async function decide(
  apiBase: string,
  policy: string,
  context: unknown,
) {
  const res = await fetch(`${apiBase}/v1/decide`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ policy, context }),
  });

  const text = await res.text();
  if (!res.ok) throw new Error(text);
  return JSON.parse(text);
}
