# The project

This is a demodulator for MT63-2kL, an amateur radio "digital mode".

A primary goal of this project is to begin to satisfy the authors' curiosity for the details of transferring data over radio waves. Another goal is to run the demodulator in a web page, so radio amateurs can see a bit of digital radio modes in action before installing special software.

## The mode

MT63 is a way of encoding data as an audio signal in the frequency range of the human voice. It can be transmitted and received by equipment designed for use with the human voice, such as speakers, microphones, and amateurs' FM radios.

It's not especially fast, but it uses sufficiently conservative audio frequencies, symbol rates, and error correction to work with acoustic coupling on both ends of the conversation.

It uses 64 audio-frequency carriers with differential phase-shift keying (DPSK). Those carriers may be spread across 500, 1000, or 2000 Hz. We're focusing on the 2 kHz variant.

It spreads the bits of each symbol across time, with either "short" or "long" interleaving. We're focusing on the "long" variant.

In total, we're targeting "MT63-2kL", which sees use over line-of-sight FM links on the 2-meter and 70-cm bands to transfer small messages such as ICS-213 forms, which may be around 500 bytes each.

Given the focus on audio sent over FM links, we're not trying to correct for the sort of mis-tuning one might experience with SSB, where all of the carriers would be shifted up or down by some number of Hz.

## Additional technical details

In the 2 kHz variant, the 64 carriers are separated by 31.25 Hz with the first centered at 500 Hz. There are 20 symbols per second, one every 50 ms.

### Alternating carriers

Looking at sample audio generated by Fldigi, the odd numbered and even numbered carriers alternate in time. The even numbered carriers will be at their maximum amplitude at the same time that the odd numbered carriers are at their minimum amplitude (zero). Half a symbol period later, those roles will be reversed.

### DPSK

Each carrier is sent with some phase. When the envelope of that carrier dips to zero and then returns to its maximum amplitude, that phase may be inverted. We detect whether the phase of the carrier changed, thus decoding one bit.

For a given carrier, we expect to find a phase reversal, or lack of a phase reversal, every 50 ms symbol period.

We copy the signal, shift it in time, and subtract it from itself. We look at a time range no larger than 50 ms. We calculate the magnitude and phase at a particular carrier frequency of the "before" signal, the "after" signal, and the combined signal.

The carriers are sent with an envelope, so if our time range is centered near a peak of the envelope we'll get a relatively large magnitude for each of the "before" and "after" signals.

To make the phase math easier, we choose the time shift to be an integer multiple of the carrier's period. We compare the magnitude that the independent signals have at the carrier frequency with the magnitude of the combined signal.

When there's no phase shift, we see only a small magnitude: the wave subtracted from an in-phase copy of itself leaves little or nothing. A large magnitude, especially one that's close to the sum of the "before" and "after" magnitudes, indicates a 180° phase shift.

### The 32 ms window

Note that the 31.25 Hz separation means there's a cycle every 32 ms: in a window 32 milliseconds long, we can fit 16 complete cycles of the 500 Hz carrier, 17 complete cycles of the 531.25 Hz carrier, 18 complete cycles of the third carrier, up through 79 complete cycles of the 64th carrier at 2468.75 Hz.

We can use this to decrease the amount of phase shift math we need to do.

### Decoding the symbol clock

We can try every quarter symbol, 12.5 ms for the 2 kHz variant, looking for a time of relatively high power in the odd carriers offset by a half-symbol from a time of relatively high power in the even carriers.

### Now we have bits

The result of the above work is to convert a stream of audio data into a set of 64 bits every 50 ms, 1280 bits or 160 bytes per second. We may wish to retain a level of certainty for each bit, but we'll delay that complexity until it seems necessary and useful.

### The Augmented Hadamard code

The input to an MT63 modulator is 7-bit ASCII characters. Per the available technical references, each is encoded as 64 bits via an Augmented Hadamard code. Or Walsh function. Or Walsh–Hadamard code.

Each of the possible 64-bit symbols is orthogonal to the others (or the inverse of another), meaning we'd need to change at least half of the bits to move from one code to another. If there's an error in up to a quarter of the bits, we'll see that one of the encodings is closer than the others, allowing us to recover the original 7 bits that selected the 64-bit symbol.

### Interleaving

The available technical references describe MT63 as "interleaving" the bits of each 64-bit symbol across multiple ticks of the symbol clock. With MT63-2kL, the interleaving is "long" rather than "short".

We have not found documentation of the details of that interleaving. Instead we intend to employ a chosen-plaintext attack on the protocol, asking Fldigi to encode data with specific bit patterns and then analyzing the result.

### Maybe that's all?

We'll see if we get intelligible data.

## Some references

ARRL MT-63 technical description: https://www.arrl.org/mt-63

Fldigi mode description: http://www.w1hkj.com/FldigiHelp/mt63_page.html

DPSK demodulator design: https://www.youtube.com/watch?v=tNvNyg5Omq0

(Augmented) Hadamard code: https://en.wikipedia.org/wiki/Hadamard_code
