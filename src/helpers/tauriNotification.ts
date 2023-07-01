import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/api/notification";

/**
 * Sends a notification to the user if the permission is granted, otherwise it requests the permission
 *
 * @param title Title of the notification
 * @param body Body of the notification
 * @returns Promise<void>
 */
export async function sendTauriNotification(title: string, body: string) {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
    }

    if (permissionGranted) {
        sendNotification({ title, body });
    }
}
