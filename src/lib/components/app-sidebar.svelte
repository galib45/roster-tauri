<script lang="ts">
 import * as Sidebar from "$lib/components/ui/sidebar/index.js";
 import OrganizationSelector from "$lib/components/organization-selector.svelte";
 import {
     ClipboardList, BuildingComplexIcon,
     LayoutDashboardIcon, BriefcaseBusinessIcon,
     UsersIcon, ClockIcon, CalendarRangeIcon,
     CalendarCheckIcon, ChartNoAxesCombinedIcon,
     SettingsIcon
   } from "@lucide/svelte";

   import { useSidebar } from "$lib/components/ui/sidebar/index.js";

    import SidebarButtonWithLink from "./sidebar-button-with-link.svelte";
    import { getAppState } from "$lib/state.svelte";

    const sidebar = useSidebar();
    const appState = getAppState();
 const organizationsNavItem = {
   title: "Organizations", icon: BuildingComplexIcon, url: "/organizations"
 }

 const settingsNavItem = {
   title: "Settings", icon: SettingsIcon, url: "/settings"
 }

  const navigationItems = [
    { title: "Overview", icon: LayoutDashboardIcon, url: "/overview" },
    { title: "Positions", icon: BriefcaseBusinessIcon, url: "/positions"},
    { title: "Employees", icon: UsersIcon, url: "/employees" },
    { title: "Shifts", icon: ClockIcon, url: "/shifts"},
    { title: "Roster Templates", icon: CalendarRangeIcon, url: "/roster-templates" },
    { title: "Rosters", icon: CalendarCheckIcon, url: "/rosters" },
    { title: "Reports", icon: ChartNoAxesCombinedIcon, url: "/reports" },
  ];
</script>

<Sidebar.Root collapsible="icon">
 <Sidebar.Header>
     <!-- App branding -->
    <Sidebar.Menu>
    <Sidebar.MenuItem>
        <Sidebar.MenuButton size="lg">
        <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
            <ClipboardList class="size-4" />
        </div>

        <div class="flex flex-col gap-0.5">
            <span class="font-semibold">Roster Manager</span>
            <span class="text-xs text-muted-foreground">
            Workforce Management
            </span>
        </div>
        </Sidebar.MenuButton>
    </Sidebar.MenuItem>
    </Sidebar.Menu>
    <Sidebar.Menu>
    <Sidebar.MenuItem>
        <SidebarButtonWithLink item={organizationsNavItem}/>
    </Sidebar.MenuItem>
    </Sidebar.Menu>
    <!-- Organization selector -->
    {#if sidebar.state === "expanded" && appState.hasOrganization }
        <OrganizationSelector />
    {/if}
 </Sidebar.Header>
 <Sidebar.Content>
     <!-- Workspace navigation -->
     <Sidebar.Group>
       <Sidebar.GroupLabel>WORKSPACE</Sidebar.GroupLabel>

       <Sidebar.GroupContent>
         <Sidebar.Menu>
           {#each navigationItems as item}
             <Sidebar.MenuItem>
                 <SidebarButtonWithLink item={item} />
             </Sidebar.MenuItem>
           {/each}
         </Sidebar.Menu>
       </Sidebar.GroupContent>
     </Sidebar.Group>
   </Sidebar.Content>

   <!-- Bottom navigation -->
   <Sidebar.Footer>

     <Sidebar.Menu>
       <Sidebar.MenuItem>
           <SidebarButtonWithLink item={settingsNavItem} />
       </Sidebar.MenuItem>
     </Sidebar.Menu>
   </Sidebar.Footer>

   <Sidebar.Rail />
</Sidebar.Root>
