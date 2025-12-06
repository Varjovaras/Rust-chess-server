<script lang="ts">
    import { page } from "$app/stores";
    console.log($page.error);
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
<div>
    backend not online, shuttle.rs might need a few seconds to activate from
    hiatus
</div>
