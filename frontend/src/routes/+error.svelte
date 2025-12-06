<script lang="ts">
    import { page } from "$app/stores";
    console.log($page.error);
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
<div>
    Backend server is not online. Please check that the server is running or try
    again in a few moments.
</div>
