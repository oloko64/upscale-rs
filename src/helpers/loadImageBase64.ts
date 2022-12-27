import { invoke } from "@tauri-apps/api/tauri";

/**
 * Takes a path to an image and returns a base64 encoded string of the image, can throw an error if the image is not found
 * 
 * @param path Local path to the image
 * @returns Base64 encoded string of the image
 */
export async function loadImage(path: string): Promise<string> {
  const imageBytes = await invoke("read_image_base64", {
    path: path,
  });
  return `data:image/png;base64,${imageBytes}`;
}