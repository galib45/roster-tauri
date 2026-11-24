<script lang="ts">
  import './layout.css';
  import { ModeWatcher } from "mode-watcher";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";

  import { getAppState, setProjectContext } from "$lib/state.svelte";
    import { onMount } from 'svelte';

  let appState = getAppState();
  setProjectContext(appState);
  onMount(() => {
      appState.load();
  });

  const { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
 <AppSidebar />
 <main class="relative flex min-h-screen w-full items-center justify-center bg-background p-4">
    <Sidebar.Trigger class="absolute left-0 top-0 z-50" />
    {#if appState.loading}
          <p>Loading project...</p>
        {:else if appState.error}
          <div class="text-center">
            <p class="text-destructive">{appState.error}</p>
            <button onclick={appState.load}>
              Retry
            </button>
          </div>
        {:else}
          {@render children?.()}
        {/if}
 </main>
</Sidebar.Provider>
