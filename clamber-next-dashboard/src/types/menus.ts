export interface MenuItem {
    label: string;
    path: string;
    icon?: string;
    permission?: string; // 权限标识
    children?: MenuItem[];
}
