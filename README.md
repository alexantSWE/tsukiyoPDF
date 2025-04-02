# 🌙 tsukiyoPDF: Yet Another PDF Reader (This Time in Rust! 🦀)

🚀 **Tagline:** We're aiming for "super fast." Right now, we're settling for "it compiles... sometimes." Dark Mode support? Sure, that's on the wishlist, right after "actually rendering PDFs reliably." 😉

---

## The Questionable Arsenal We've Assembled:

*   **FLTK (UI):** Because who needs modern UI toolkits when you have something that peaked aesthetically in 1998? It's *fast*, *statically linked*, and blessedly *consistent* (it'll look equally out-of-place on *all* platforms!). Think of it as building a spaceship cockpit with LEGO Duplo. Cross-platform! Minimalist! Barely any safety rails!
*   **PDFium-render (Backend/Renderer):** The mysterious black box that *actually* does the hard work. We poke it with sticks (via Rust bindings) and hope it spits out pixels instead of cryptic error messages.
*   **lopdf:** Our valiant attempt to wrangle PDFium and pretend we understand the eldritch horror that is the PDF specification. It parses! It processes! It probably introduces subtle bugs we won't find for months!

---

## "Seriously, Dude. FLTK?" - A Defense (of sorts)

Look, **yeah**, FLTK. Why the heck not?

*   **Speed? Check.** (Allegedly. We haven't benchmarked anything but our caffeine intake).
*   **Tiny Binary? Check.** (Fits on a floppy disk! If you still have one. And a drive. And remember what they are).
*   **Cross-Platform Consistency? Check.** (Consistently looks like it escaped a time capsule, regardless of OS).
*   **The Real Reason?** FLTK is like a programming challenge from a bygone era. It gives you *nothing*. No fancy abstractions, no helpful hand-holding. It practically *begs* you to reinvent the wheel, implement bafflingly complex custom widgets for simple tasks, and generally make things harder than they need to be. It's artisanal suffering, handcrafted just the way I like it. *sips lukewarm coffee*

---

## 🤔 What's in a Name? Existential Musings on "tsukiyoPDF"

The name derives from 月夜 (Tsukiyo) - Japanese for "moonlit night" 🌙✨. It evokes a sense of calm, beauty, and serenity.

Which is ironic, given the current state of the code resembles more of a frantic, caffeine-fueled coding session during a power outage with only a flickering candle for light. But hey, aspirations!

but not really, more like , I wanted something cool sounding that related to the final destionation this project plans to arrive at(which is, dark mode support, hence the moon thingie in the name)
---

## 💡 Genesis: Why This Digital Masochism?

1.  **Learning Rust by Trial and Error (Mostly Error):** Gotta jump on the Rust bandwagon 🦀, right? It's memory-safe! It's fast! It powers the very fabric of the modern Linux universe 🐧 (or so the internet tells me). What better way to learn than by building something slightly too ambitious?
2.  **NightPDF Looked Cool:** Saw NightPDF, thought, "I can do that!" (Spoiler: It's harder than it looks, and this project is now veering wildly off course).
3.  **Avoiding the Bloat Monsters:** A desperate, possibly futile attempt to sidestep the siren call of Electron and Tauri 💻🚫. My RAM thanks me, my sanity... less so.
4.  **Embracing the Noob Lyfe:** 🍼 I am but a humble beginner, flailing wildly in the sea of systems programming. This project is my paddle (or maybe just a splintered piece of driftwood).

---

## 🚨 **DISCLAIMER: PERPETUAL ALPHA ZONE** 🚨

This project currently resides in the "Educational / What Was I Thinking?" phase and has built a cozy little nest there. It might *never* leave. Expect bugs, crashes, features that vanish mysteriously, and the occasional existential panic commit. Use at your own peril (and amusement).

---

## 🛠️ The Dream Board: Features We Might Implement (If the Stars Align)

**(Basic Tier - Stuff That *Should* Be Easy But Probably Isn't)**

*   📂 **Open PDF:** Click file -> See PDF. The absolute bare minimum. Success not guaranteed.
*   <0xF0><0x9F><0x94><0x82> **Close PDF:** Preferably without crashing the whole application. (Stretch goal: Releasing file handles properly).
*   ⚡ **Hardware Acceleration:** Because rendering JPEGs from 2003 should *really* leverage that shiny new RTX card, right? We *need* the speed! (Also, this sounds impressive).
*   🖥️ **Hardware Acceleration (Again):** Look, it's *important*, okay? Might not be basic, but my CPU fan is already judging me.
*   🚫 **No Generic Wayland Logo:** A personal vendetta. If this thing ever runs on Wayland natively, it better have a damn icon. Future me is already annoyed.

**(Advanced Tier - AKA "When Pigs Fly" / "After I Win the Lottery")**

*   ✏️ **Annotations/Highlighting:** Let's draw squiggles on important documents! Maybe even *save* them? Woah there, steady on.
*   🗂️ **Tabbed Interface:** Because who only reads *one* PDF at a time? This is probably harder than rendering itself.

---

## 🤝 Want to Dive Into This Glorious Mess? (Contributing)

This project is currently somewhere between primordial soup 🦠 and a shaky toddler taking its first steps 🐣. The future is... let's call it "malleable" 🔮.

If you're feeling brave, foolish, or just incredibly bored:

1.  **Fork it:** You know, the usual GitHub dance.
2.  **Create a Branch:** Name it something descriptive like `fix-the-dumpster-fire` or `maybe-add-a-button`.
3.  **Make Changes:** Try not to break *everything*. (Good luck!).
4.  **Submit a PR:** 🧑‍💻 Offer your sacrifice to the code gods. Explain what you did and why you thought it was a good idea.
5.  **Open Issues:** 💬 Found a bug? Have a terrible idea? Share it! Let's commiserate.

**Issue Templates?** Ha! Maybe later. For now, just wing it. 🙌

---

## 📜 License: The Fine Print Nobody Reads

**MIT License** 📝

(Basically: Do whatever you want with this code, but if it summons Cthulhu, deletes your homework, or sets your computer on fire, don't come crying to me. It's free! You get what you pay for.)

for some reason, I decided to ruin my awesomly written README file and tell our boy Gemini experimental to rewrite it in a weirdly satirical way

cheers!
