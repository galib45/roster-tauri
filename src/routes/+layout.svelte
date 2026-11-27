<script lang="ts">
  import './layout.css';
  import { ModeWatcher } from "mode-watcher";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";

  import { AppState, setAppState } from "$lib/state.svelte";
  import { onMount } from 'svelte';
    import { Button } from '$lib/components/ui/button';

  let appState = new AppState;
  setAppState(appState);
  onMount(() => appState.load());

  const { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
 <AppSidebar />
 <main class="flex min-h-screen w-full bg-background py-4">
    <!-- <Sidebar.Trigger class="fixed left-(--sidebar-width) top-0 z-50 rounded-r-md border border-l-0 bg-background/90 shadow-md backdrop-blur-md" /> -->
    {#if appState.loading}
        <p>Loading project...</p>
    {:else if appState.error}
        <div class="text-center w-full space-y-2">
            <p class="text-destructive">{appState.error}</p>
            <Button onclick={appState.load}>Retry</Button>
        </div>
    {:else}
        {@render children?.()}
    {/if}
 </main>
</Sidebar.Provider>
