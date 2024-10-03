+++
date = 2024-10-03
title = "A Toy with an SDK"
+++

Have you noticed how much some kids love to push buttons or flip a light switch on and off? My son loved it so much I built him a button box using an Arduino, 10 mechanical keyboard switches with RGB lights, and a speaker. Here is a video of him learning his colors, the first activity I built for him.

<iframe style="float: left; margin: 10px; max-width: 315px; width: 100%; height: 560px"
src="https://www.youtube.com/embed/MyZ18ibgZGk"
title="YouTube video player"
frameborder="0"
allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
allowfullscreen></iframe>

A family friend, unaware of the device, commented on how many colors our son knew and how rare that is for a 1-year-old. After taking a break during a move and COVID, I kept building new activities (also called apps) for him and my nephews.

I wrote an activity where it just counts how many buttons have been pressed, says the number and color codes the digits like a resistor’s color bands. Super simple. It took less than 30 minutes to code up with the libraries I developed, but I was surprised how much kids liked it. I then made a variant of the app that displays the number on the buttons in binary digits instead of decimal and then watched as my 7 year old nephew learned binary. I don’t think it would be worth making a dedicated toy for either of those activities, but a toy with an SDK and maybe over a hundred apps targeted to different stages of a child's growth? That's what's kept me so excited about this idea for so long.

It's fun as a programmer too. Instead of worrying about UI frameworks for 4k monitors, which dependencies to use and everything else that goes into modern app development, it's just 10 lights, 10 buttons and a speaker. And there is actually a lot you can do with just that. This platform is about as simple as a "computer" can be while still being multi-purpose.

I have already built 30 apps ranging from arcade style games (e.g. simon says repeat the pattern, reaction time games, bop-a-mole), to a noise machine with 10 different sounds that can be toggled, to streaming kids' podcasts over WiFi. What really excites me is the potential for a community where people can build and share their own apps. I am going to launch an SDK so people can make their own. Some initial details are on our [Github page](https://github.com/10buttons/).

It has been 2.5 years working on boppo full-time. During that time, I learned enough electronics to design a PCB for the prototypes with an ESP32. I started from the basics. For example, I returned a lab power supply to Amazon because I thought I was supposed to hook my device to positive and ground (should have been positive and negative). I also spent 2 weeks and bought an oscilliscope to realize that my I2C cables were just too long in a prototype. And don't ask me how many different speaker drivers I bought looking for good sound. I have gone through at least 6 iterations.

Another challenge I faced was building an efficient audio playback system for the ESP32 chip in Rust. None of the existing libraries worked well in the constrained environment, so I spent a few solid weeks writing and open-sourcing one: [awedio](https://github.com/10buttons/awedio).

Earlier this year, my wife joined me to help. She and a friend branded it [boppo](https://boppo.com/) which I love. We have built faceplates that go on top of the buttons, which the tablet detects with NFC. This gives the buttons new meaning and we are building out multiple activities for each faceplate.

<img style="width: 100%; max-height: 432px; max-width: 500px;" src="/toy-with-an-sdk/boppo_plates.webp" >


I have been marketing boppo as a “screenless tablet” because I think that best represents its capabilities (but there have been vocal critics of that wording). A toy with an SDK might be more accurate. Other descriptions or startup analogies that come to mind are:

* A mechanical keyboard with a microcontroller and a speaker
* A screenless educational game console
* Flipper Zero but for kids and learning not security hacking
* Stream Deck without the LCD but with a battery and Wifi

 Our project is on [Kickstarter]. If what I am working on interests you, we would [love your support][Kickstarter].

<a href="https://www.kickstarter.com/projects/boppo/boppo-the-screenless-tablet?ref=7dg1xx">
<img style="border: solid black 1px; width: 100%; max-width: 800px" src="/toy-with-an-sdk/Kickstarter_Screenshot.webp">
</a>

[Kickstarter]: https://www.kickstarter.com/projects/boppo/boppo-the-screenless-tablet?ref=7dg1xx
