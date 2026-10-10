/**
 * Newspaper press conductor - the full prints-the-page show.
 *
 * One GSAP timeline assigns every element a slot in document order (the
 * "press wave", top to bottom):
 *
 *   - [data-press-wire] blocks rewind into register: displaced low,
 *     tilted, opacity 0 -> home, fast launch / long glide. Blocks that
 *     are themselves letter-scrambled get a smaller throw since their
 *     letters do the detailing.
 *   - [data-newspaper-scramble] containers (masthead, featured story,
 *     post title) carry server-baked letter spans (.np-sc with glyph
 *     streams from src/scramble.rs). Each letter is scattered by a
 *     transform (layout-safe: transforms never move layout), cycles
 *     decoy glyphs, and lands at its own slot + data-d schedule.
 *   - [data-word-scatter] pieces (topline titles, deck, byline) are
 *     split into inline-block words at runtime and flown home the same
 *     way - no glyphs, too small to earn the cipher and too numerous
 *     to afford it.
 *
 * After letters lock, the word wrappers keep their places and top-flight
 * kerning returns inside each word - the settled page stays visually
 * continuous with the flying one, no wholesale reflow at the end.
 *
 * No JS, no GSAP, or reduced motion -> static page (np-press never
 * starts hiding anything).
 */
(() => {
    const root = document.documentElement;

    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
        root.classList.remove("np-press");
        return;
    }
    if (!window.gsap) {
        root.classList.remove("np-press");
        return;
    }

    /**
     * TIMING SPEC - the single table every animated piece reads from.
     * Server-side counterparts (src/scramble.rs):
     *   MASTHEAD_SPREAD_MS 1100 -> letter spread, masthead
     *   HEADLINE_SPREAD_MS  700 -> letter spread, headlines
     * CSS counterparts: .newspaper-scrolled-in flight matches
     * block.flight; .newspaper-dN delay stride matches wave.spacing.
     * Keep the pairs in sync when pacing changes.
     */
    const SPEC = {
        wave: { start: 0.5, spacing: 0.06 }, // press wave, top to bottom
        block: {
            flight: 0.85, // matches .newspaper-scrolled-in (editions)
            throwY: [40, 90], // px - full-weight blocks
            throwX: [-16, 16],
            tilt: [-2.5, 2.5],
            throwYLight: [10, 24], // px - blocks whose letters detail them
            throwXLight: [-6, 6],
            tiltLight: 1.5,
        },
        rule: { draw: 0.85 },
        letter: {
            flight: 0.5, // matches word.flight: one rhythm for text pieces
            settle: 0.25, // CSS np-letter-press
        },
        word: { flight: 0.5, spacing: 0.035 },
    };

    const TICK_MS = 40; // glyph cadence
    const T0 = SPEC.wave.start;

    const borders = document.querySelectorAll(".newspaper-masthead-border");
    const wires = Array.from(document.querySelectorAll("[data-press-wire]"));
    const containers = Array.from(
        document.querySelectorAll("[data-newspaper-scramble]")
    );

    // Document-order slot assignment: blocks and letter containers share
    // one ordered list, so the sweep is strictly top-to-bottom.
    const byDoc = (a, b) =>
        a.compareDocumentPosition(b) & Node.DOCUMENT_POSITION_FOLLOWING
            ? -1
            : 1;
    const slotOf = new Map(
        [...wires, ...containers]
            .sort(byDoc)
            .map((el, i) => [el, T0 + i * SPEC.wave.spacing])
    );

    const tl = gsap.timeline({ defaults: { ease: "power4.out" } });

    // --- Block rewinds ---
    wires
        .filter((w) => !w.classList.contains("newspaper-masthead-border"))
        .forEach((w) => {
            const detailed = w.hasAttribute("data-newspaper-scramble");
            const throwY = detailed
                ? SPEC.block.throwYLight
                : SPEC.block.throwY;
            const throwX = detailed
                ? SPEC.block.throwXLight
                : SPEC.block.throwX;
            const tilt = detailed
                ? SPEC.block.tiltLight
                : gsap.utils.random(...SPEC.block.tilt);

            tl.fromTo(
                w,
                {
                    opacity: 0,
                    y: gsap.utils.random(...throwY),
                    x: gsap.utils.random(...throwX),
                    rotation: tilt,
                },
                {
                    opacity: 1,
                    x: 0,
                    y: 0,
                    rotation: 0,
                    duration: SPEC.block.flight,
                },
                slotOf.get(w)
            );
        });

    // The double rule draws across instead of flying; reclaim its
    // pre-hide opacity, then draw.
    borders.forEach((b) => {
        const slot = slotOf.get(b) ?? T0;
        tl.set(b, { opacity: 1 }, slot);
        tl.fromTo(
            b,
            { scaleX: 0 },
            { scaleX: 1, duration: SPEC.rule.draw, transformOrigin: "left center" },
            slot
        );
    });

    // --- Letter scatter + cipher (masthead, featured story, post title) ---
    const players = [];

    containers.forEach((c) => {
        const letters = Array.from(c.querySelectorAll(".np-sc[data-g]"));
        if (!letters.length) return;

        const slot = slotOf.get(c) ?? T0;
        const masthead = c.classList.contains("newspaper-masthead");

        // Make the container visible at its slot; letters are already
        // scattered (transforms only - layout untouched) from t=0.
        tl.set(c, { opacity: 1 }, slot);

        letters.forEach((el) => {
            const stream = el.dataset.g;
            if (!stream) return;
            el.textContent = stream[0];
            const reveal = slot + (Number(el.dataset.d) || 0) / 1000;

            const scale = masthead ? 1 : 2.4;
            gsap.set(el, {
                x: gsap.utils.random(-9, 9) * scale,
                y: gsap.utils.random(3, 14) * scale,
                rotation: gsap.utils.random(-2, 7),
            });
            tl.to(
                el,
                { x: 0, y: 0, rotation: 0, duration: SPEC.letter.flight },
                reveal
            );
            // The glyph driver may not release the page (and unwrap) until
            // every letter is truly home, not merely on its way home.
            players.push({
                el,
                reveal: reveal * 1000,
                deadline: (reveal + SPEC.letter.flight) * 1000,
            });
        });
    });

    // --- Word scatter (topline titles, deck, byline) ---
    const wordPieces = Array.from(
        document.querySelectorAll("[data-word-scatter]")
    );
    const wordSlots = [];
    const CHUNK = SPEC.word.spacing;

    wordPieces.forEach((el) => {
        const text = el.textContent.trim();
        if (!text) return;

        const words = text.split(/\s+/);
        const frag = document.createDocumentFragment();
        words.forEach((word, i) => {
            if (i > 0) frag.append(" ");
            const span = document.createElement("span");
            span.className = "np-wd";
            span.textContent = word;
            frag.append(span);
        });
        el.replaceChildren(frag);

        // Slot: the element itself when wired, else its enclosing block
        const host = el.hasAttribute("data-press-wire")
            ? el
            : el.closest("[data-press-wire]");
        const base = slotOf.get(host) ?? T0;

        gsap.set(piecesOf(el), {
            x: () => gsap.utils.random(-10, 10),
            y: () => gsap.utils.random(4, 20),
            rotation: () => gsap.utils.random(-4, 4),
        });

        wordSlots.push({ words: piecesOf(el), container: el, base });
    });

    wordSlots.forEach((piece) => {
        tl.set(piece.container, { opacity: 1 }, piece.base);
        piece.words.forEach((word, i) => {
            tl.to(
                word,
                { x: 0, y: 0, rotation: 0, duration: SPEC.word.flight },
                piece.base + i * CHUNK
            );
        });
    });

    function piecesOf(el) {
        return Array.from(el.querySelectorAll(".np-wd"));
    }

    // After flight, kerning must return inside words: letters collapse into
    // plain text, but the word wrappers stay put, so the unwrap shift is
    // minimal instead of a whole-line reflow.
    function unwrapLetters(container) {
        container
            .querySelectorAll("[data-g]")
            .forEach((el) => el.removeAttribute("data-g"));
        container.querySelectorAll(".np-w").forEach((wordEl) => {
            const word = Array.from(wordEl.querySelectorAll(".np-sc"))
                .map((el) => el.dataset.c)
                .join("");
            if (word) wordEl.textContent = word;
        });
        // Word wrappers (.np-w, plain inline now) remain in place.
    }

    if (containers.length) {
        let tick = 0;
        const press = setInterval(() => {
            tick += 1;
            const now = performance.now();
            let pending = false;

            players.forEach((p) => {
                if (now < p.reveal) {
                    pending = true;
                    const stream = p.el.dataset.g;
                    if (stream) p.el.textContent = stream[tick % stream.length];
                } else if (p.el.dataset.g) {
                    p.el.textContent = p.el.dataset.c;
                    p.el.removeAttribute("data-g");
                    p.el.classList.add("np-s-on");
                }
                if (now < p.deadline) pending = true;
            });

            if (!pending) {
                clearInterval(press);
                containers.forEach(unwrapLetters);
            }
        }, TICK_MS);
    } else if (wordSlots.length) {
        // No letter work on this page; word pieces keep their spans
        // (no end-of-show reflow). Nothing to clean up.
        return;
    }
})();
