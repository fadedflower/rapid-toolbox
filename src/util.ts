import { ConfirmationOptions } from "primevue/confirmationoptions";
import { Theme, ThemeColor } from "./types";

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

export function getThemeColorCssValue(color: ThemeColor) {
    switch (color.type) {
        case "RGB":
            return `rgb(${color.r}, ${color.g}, ${color.b})`;
        case "HSL":
            return `hsl(${color.h}, ${color.s}%, ${color.l}%)`;
    }
}

export function getThemeStyle(theme: Theme) {
    switch (theme.type) {
        case "Solid":
            return { backgroundColor: getThemeColorCssValue(theme.color) };
        case "LinearGradient":
            return { background: `linear-gradient(to top right, ${getThemeColorCssValue(theme.from)}, ${getThemeColorCssValue(theme.to)})` };
        case "RadialGradient":
            return { background: `radial-gradient(circle, ${getThemeColorCssValue(theme.from)} 0%, ${getThemeColorCssValue(theme.to)} 100%)` };
    }
}

export function cloneTheme(theme: Theme): Theme {
    switch (theme.type) {
        case "Solid":
            return { type: "Solid", color: { ...theme.color } };
        case "LinearGradient":
            return { type: "LinearGradient", from: { ...theme.from }, to: { ...theme.to } };
        case "RadialGradient":
            return { type: "RadialGradient", from: { ...theme.from }, to: { ...theme.to } };
    }
}

export function preventDndAction(event: DragEvent) {
    event.preventDefault();
    if (event.dataTransfer) {
        event.dataTransfer.dropEffect = "none";
    }
}