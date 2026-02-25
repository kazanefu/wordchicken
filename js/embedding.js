import { pipeline } from "@xenova/transformers";

let embedder;

export async function embed(text) {
  if (!embedder) {
    embedder = await pipeline(
      "feature-extraction",
      "Xenova/all-MiniLM-L6-v2"
    );
  }

  const output = await embedder(text, { pooling: "mean", normalize: true });
  return new Float32Array(output.data);
}