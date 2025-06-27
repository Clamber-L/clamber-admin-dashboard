// mock-api/menus.ts
import type { MenuItem } from '@/types/menus';

export const mockMenus: MenuItem[] = [
    {
        label: 'Dashboard',
        path: '/',
        icon: 'dashboard',
    },
    {
        label: 'Users',
        path: '/users',
        icon: 'users',
        permission: 'view_users',
        children: [
            { label: 'User List', path: '/users/list', permission: 'view_users' },
            { label: 'Add User', path: '/users/create', permission: 'create_user' },
        ],
    },
    {
        label: 'Settings',
        path: '/settings',
        permission: 'admin',
        children: [
            { label: 'Profile', path: '/settings/profile' },
            { label: 'Security', path: '/settings/security', permission: 'admin' },
        ],
    },
];
