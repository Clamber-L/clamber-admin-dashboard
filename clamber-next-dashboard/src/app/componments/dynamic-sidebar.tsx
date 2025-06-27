'use client';

import type { LucideIcon } from 'lucide-react';
import type { AppRouterInstance } from 'next/dist/shared/lib/app-router-context.shared-runtime';

import { LayoutDashboard, Users } from 'lucide-react';
import { usePathname, useRouter } from 'next/navigation';
import React, { useEffect } from 'react';

import type { MenuItem } from '@/types/menus';

import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from '@/app/_components/shadcn/ui/collapsible';
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarMenuSub,
} from '@/app/_components/shadcn/ui/sidebar';
import { sidebarStore } from '@/store/sidebar-store';

const iconMap: Record<string, LucideIcon> = {
    'layout-dashboard': LayoutDashboard,
    users: Users,
};

const items: MenuItem[] = [
    {
        label: 'Dashboard',
        icon: 'layout-dashboard',
        path: '/',
    },
    {
        label: 'Management',
        icon: 'users',
        path: 'user',
        children: [
            {
                label: 'Users',
                path: '/user/users',
            },
            {
                label: 'Roles',
                path: '/user/roles',
            },
        ],
    },
    {
        label: 'Test',
        icon: 'users',
        path: 'user',
        children: [
            {
                label: 'Users',
                path: '/user/users',
            },
            {
                label: 'Roles',
                path: '/user/roles',
            },
        ],
    },
];

const renderMenu = (items: MenuItem[], navigate: AppRouterInstance) => {
    const { openKey, setOpenKey } = sidebarStore();

    return items.map((item) => {
        const key = item.label;

        const Icon = iconMap[item.icon || 'layout-dashboard'];

        if (item.children?.length) {
            return (
                <Collapsible key={item.label} open={openKey === key}>
                    <SidebarMenuItem>
                        <CollapsibleTrigger asChild>
                            <SidebarMenuButton onClick={() => setOpenKey(key)}>
                                {Icon && <Icon className="mr-2 h-4 w-4" />}
                                {item.label}
                            </SidebarMenuButton>
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <SidebarMenuSub>{renderMenu(item.children, navigate)}</SidebarMenuSub>
                        </CollapsibleContent>
                    </SidebarMenuItem>
                </Collapsible>
            );
        } else {
            return (
                <SidebarMenuItem key={item.label}>
                    <SidebarMenuButton onClick={() => navigate.push(item.path!)}>
                        {Icon && <Icon className="mr-2 h-4 w-4" />}
                        {item.label}
                    </SidebarMenuButton>
                </SidebarMenuItem>
            );
        }
    });
};

export default function DynamicSidebar() {
    const navigate = useRouter();
    const pathname = usePathname();

    // 工具函数：从当前路径找出应该展开哪些 key
    const collectOpenKeysFromPath = (items: MenuItem[], pathname: string): string => {
        for (const item of items) {
            if (item.path && pathname.startsWith(item.path)) {
                return item.label;
            }
        }
        return '/';
    };

    useEffect(() => {
        const defaultKeys = collectOpenKeysFromPath(items, pathname);
        sidebarStore.getState().setOpenKey(defaultKeys);
    }, [pathname]);

    return (
        <Sidebar collapsible="icon">
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>Application</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>{renderMenu(items, navigate)}</SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    );
}
