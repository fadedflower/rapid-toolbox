import { ConfirmationOptions } from "primevue/confirmationoptions";

type MessageDialogIcon = "info" | "warning" | "error" | "success";
interface ConfirmInstance {
    require: (option: ConfirmationOptions) => void;
    close: () => void;
};

export function messageDialog(confirm: ConfirmInstance, header: string, message: string, icon: MessageDialogIcon = "info", onClose?: () => void) {
    let iconClass = "";
    switch (icon) {
        case "info":
            iconClass = "pi pi-info-circle";
            break;
        case "warning":
            iconClass = "pi pi-exclamation-triangle";
            break;
        case "error":
            iconClass = "pi pi-times-circle";
            break;
        case "success":
            iconClass = "pi pi-check-circle";
            break;
    }

    confirm.require({
        message,
        header,
        icon: iconClass,
        acceptProps: {
            label: 'OK',
            size: 'small'
        },
        rejectClass: 'hidden',
        accept() {
            if (onClose) {
                onClose();
            }
        },
        onHide() {
            if (onClose) {
                onClose();
            }
        }
    });
}