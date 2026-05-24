---
title: "Absolute Cinema: How I Built a Production-Grade CI/CD & R2 Backup Pipeline for $0 on a 2GB Potato Server"
date: "2026-05-24"
formatted_date: "May 24, 2026"
author: "Mega Nugraha"
summary: "Architecting a highly coherent envelope: zero-downtime CI/CD, automated off-site backups, and extreme server tuning on a $5 potato server."
tags: ["DevOps", "CI/CD", "Linux", "Caddy", "Laravel"]
published: true
---

I have a confession to make. I used to look at modern enterprise deployment pipelines and feel a profound sense of exhaustion. Every time someone suggests spinning up Kubernetes, configuring complex Jenkins pipelines, or running heavy GitHub Actions runner daemons on a production server just to deploy a standard multi-tenant app, I feel like a fraud. 

Under the hood of most "scalable" corporate stacks is a massive network of over-engineered duct tape designed to burn hundreds of dollars a month just to move files from point A to point B. 

I don't have that kind of budget. What I do have is a $5-a-month VPS in some Singapore datacenter, 2GB of RAM, a 2-core processor, and an unhealthy amount of spite. 

The server is named `akwoaowkaowkokwa` (an Indonesian keyboard smash, which represents the exact sound I make when my database crashes at 2 AM). I wanted a full-on, zero-downtime CI/CD pipeline, automated off-site backups, and high-stakes deployment flows—all running on this potato server for exactly zero pennies. 

To make this happen on a machine the size of a smartwatch, I had to architect a highly coherent envelope. Here is the blueprint of how it all hangs together—from incoming request security, through extreme server tuning, to Git-driven atomic deployments and safe, non-destructive exfiltration.

---

### Phase 1: The Cloudflare Security Envelope

Before a single byte of my application code compiles, I need to make sure `akwoaowkaowkokwa` is invisible to the public internet. I don’t want script kiddies or automated port scanners discovering the raw IP of the server. 

To achieve this, the server’s UFW (Uncomplicated Firewall) rules are configured to drop all incoming TCP traffic on ports 80 and 443 **unless** it originates from Cloudflare's official IP ranges. If you attempt to access the server directly via its IP, it will refuse to talk to you.

But this introduces a classic backend problem. When you proxy all traffic through Cloudflare, your web server (Caddy) sees every single incoming request as originating from a Cloudflare proxy IP. If a bad actor attempts to brute-force a login, your Laravel application’s rate-limiter will see the Cloudflare IP as the attacker and lock out Cloudflare, effectively taking your own site down for everyone.

Rather than installing heavy C extension modules or messing with complex system-level Caddy header configurations to extract the real user IP, we solved this securely inside the PHP application layer using a custom `TrustCloudflare` middleware.

*   **Surgical Trust:** Instead of hardcoding a static list of Cloudflare IPs in our application config (which Cloudflare updates periodically, breaking your setup), the middleware dynamically trusts the *connecting IP* as a trusted proxy. 
*   **IP Extraction:** It intercepts `CF-Connecting-IP` (which is safely injected by Cloudflare and cannot be spoofed by clients because the firewall blocks direct non-Cloudflare traffic) and normalizes it.

Now, your application's rate-limiters, logs, and audit trails see the real client's IP, completely transparently, while keeping the physical server locked in a secure fortress.

---

### Phase 2: The $5 Server Tuning Masterclass

Tuning a 2GB VPS is like packing for a two-week vacation using only a school backpack. You don't bring your favorite heavy boots; you bring one pair of versatile sneakers and pack with military precision. 

Most default database and PHP-FPM configurations assume you are running on an enterprise server with 32GB of RAM. If you leave them at defaults, MySQL will happily allocate 1.5GB of RAM to caches and indexes, and PHP-FPM will spawn 50 workers, leaving the entire OS to starve. 

So, we put both of them on a strict, clinical leash.

#### 1. Capping the Database (MySQL)

The single most important database setting on a small VPS is `innodb_buffer_pool_size`. By default, MySQL wants to hold your entire index universe in memory. We shut that down in our configuration by limiting the buffer pool to `256M`. This ensures MySQL’s actual memory footprint (RSS) stays locked at a comfortable `269 MB`. For a standard operational app with low-to-medium concurrent active sessions, this is plenty of space to keep your indexes warm without starving the rest of the server.

#### 2. Restricting the Workers (PHP-FPM)

PHP-FPM workers are memory hogs. Each worker consumes about 30MB to 50MB of RAM. If your system spawns workers dynamically without a cap, a sudden burst of traffic will spike your memory and crash the server.

We configured a highly aggressive, reactive pool, capping `max_children` to 5. At this limit, PHP-FPM is physically restricted to consuming a maximum of `250 MB` of RAM, even under a massive DDoS attack. If 100 requests hit the server, they will wait in a fast queue in Caddy and execute sequentially in milliseconds rather than spawning 100 workers and killing the machine.

Combine this with OPcache configured to load the entire framework bytecode directly into a shared `128MB` memory block. Now, all 5 PHP workers read from the same compiled shared memory block, keeping their individual footprints incredibly lean.

#### 3. The 8GB Swap File Safety Net

Bare-metal systems on small resources must have a swap space. If a heavy transaction (such as generating a massive Excel report with Spatie SimpleExcel) temporarily spikes memory, the Linux kernel needs a safety valve.

Setting up an `8GB` swap file ensures that if your RAM ever hits 100%, the OS will gracefully swap idle memory pages to the SSD rather than calling the Out-Of-Memory (OOM) killer to crash your MySQL database. 

---

### Phase 3: The Zero-Downtime Deployment Sandwich

Now that the server is secure and tuned, we need to get our code onto it. 

If you try to run `npm run build` or let Webpack/Vite compile your React assets on a 2GB RAM server, the build process itself will trigger the OOM killer. I refused to pay this "modern JS build tax" on my server. So, I offloaded it to GitHub Actions.

My deploy pipeline is split into a local/CI compiler step and an atomic server-side symlink swap.

#### 1. The Git-Fu Local/CI Builder

I wrote an orchestrator script that compiles the assets *locally* on my development machine (or inside a stateless GitHub Actions runner) and uses Git in a way that would make my CS professors weep.

By checking out a detached `HEAD` and force-adding the compiled `public/build` directory, I can push the complete production-ready build to the server without ever committing those heavy compiled assets into my permanent GitHub history. The moment the push completes, the script checks out my local branch again and discards the temporary detached state. 

#### 2. The Atomic Symlink Swap

On the server side, I have a bare Git repository waiting. When it receives the push, its `post-receive` hook extracts the code into a timestamped release directory and hands over control to a custom GNU `Makefile`.

The `Makefile` is the heart of the printing press. It link-states persistent folders, runs an optimized Composer install, and caches the framework logic with absolute cold efficiency. Caddy points strictly to the current public directory. When the new release is fully cached and prepared, the `Makefile` atomically swaps the symlink to point to the new folder. The swap happens in a microsecond. 

And because I refuse to let old release folders eat up my tiny SSD disk space, the script concludes by compressing old releases into highly-dense archives using a single thread to prevent CPU exhaustion. It squashes 63MB of code down to a microscopic 5.5MB archive. 

---

### Phase 4: The Non-Destructive R2 Exfil Pipeline

With the app securely deployed and running, I needed an off-site backup system. The destination was obvious: **Cloudflare R2**. It has S3-compatible APIs and exactly $0 in egress fees.

But backing up a 2GB bare-metal machine has two massive traps.

#### Trap 1: The Sync Wipe

If you write a naive cron job that runs `rclone sync` on your dynamic upload folder, you are living on borrowed time. 

If your local persistent storage mount drops, or if you make an accidental mistake locally, `rclone sync` will look at the empty server directory, assume you deleted the files on purpose, and immediately delete every single historical backup on Cloudflare R2 to match the empty source. 

To prevent this recursive suicide, we swapped sync for copy. The copy command checks file hashes and sizes, only uploads new or modified files, and **never** deletes anything on the backup side. If the server catches fire, R2 remains completely untouched.

#### Trap 2: Scoped Token 403s

I scoped my Cloudflare R2 API token to be highly secure—giving it **Object Read & Write** access restricted strictly to my specific bucket.

But when `rclone` runs, it tries to do a global S3 check to verify if the bucket exists before uploading. Because the API token is restricted solely to object-level operations inside the bucket, R2 blocks the root-level call, throwing a `403 Access Denied`.

The cure is a simple flag that tells `rclone` to skip the global account check and write the object directly into the bucket. By piping the database dump directly through gzip, the uncompressed database SQL never touches the SSD, saving disk lifetime. 

#### Trap 3: Permissions on Private Uploads

In Laravel, private user uploads are written directly to storage with strict ownership and permissions. Because we want to run our backups without compromising safety or running everything as `root`, we added our deployment user to the appropriate group and set group-read permissions recursively. Now, the backup process can read and copy your private uploaded PDFs and images without running as root.

---

### Phase 5: The Tight Systemd Leash

To execute this backup pipeline daily at midnight, I avoided the framework's built-in scheduler (which runs through PHP and eats FPM memory). Instead, I went full bare-metal and set up a **Systemd Service and Systemd Timer**.

Systemd allows us to enforce strict cgroups limits directly on the backup execution script. By setting aggressive nice levels and idle IO scheduling policies, we tell the Linux kernel that the backup job has the absolute lowest priority. If a web user hits the site at 2:00 AM, the backup immediately yields 100% of the CPU and disk IO to Caddy and PHP-FPM, resuming only when the server is idle. 

Enforcing a strict maximum memory limit acts as an iron-clad kill switch. If any process attempts to leak memory beyond 250MB, Systemd terminates the process immediately, protecting the main application database from ever being starved of RAM.

---

### The Verdict

Chasing corporate enterprise architectures is a sickness. People spend months configuring Kubernetes clusters and paying Vercel or AWS hundreds of dollars a month just to host simple apps. 

My entire pipeline runs on a server that costs as much as a single cup of coffee. It deploys in seconds, uses zero build-time resources on the host, compresses its history into tiny archives, and backs up its data to R2 safely and securely under a strict Systemd leash. 

It is functional, elegant, completely free of cloud provider taxes, and runs on a 2GB potato in the sky.

It is absolute cinema.
