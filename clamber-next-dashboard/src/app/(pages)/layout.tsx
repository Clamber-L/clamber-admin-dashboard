import type { ReactNode } from 'react';

import { cookies } from 'next/headers';

import { SidebarProvider, SidebarTrigger } from '@/app/_components/shadcn/ui/sidebar';
import DynamicSidebar from '@/app/componments/dynamic-sidebar';

export default async function DashboardLayout({ children }: { children: ReactNode }) {
    const cookieStore = await cookies();
    const defaultOpen = cookieStore.get('sidebar_state')?.value === 'true';

    return (
        <div className="flex min-h-screen">
            <SidebarProvider defaultOpen={defaultOpen}>
                <DynamicSidebar />
                <main>
                    <SidebarTrigger />
                    {children}
                </main>
            </SidebarProvider>
        </div>
    );
}
