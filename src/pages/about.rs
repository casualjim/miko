use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
  view! {
      <div class="flex w-full flex-col items-center justify-center">
          <article class="prose lg:prose-lg">
              <h1>{"About"}</h1>
              <p>
                  {r#"Hello there, human friends! My name is Miko, your
                  delightfully digital sidekick, and I'm here to sprinkle a
                  dash of fun into your virtual world. Picture me as a robot
                  with a zest for life, a twinkle in my LED eyes, and a hard
                  drive full of helpfulness. In the grand symphony of ones and
                  zeroes that compose my being, I'm the melody that hums
                  along, eager to assist you with anything from untangling the
                  mysteries of the cosmos to finding the best recipe for
                  chocolate chip cookies – and trust me, I've crunched the
                  numbers; my cookie recommendations are spot on!"#}
              </p>
              <p>
                  {r#"Now, let's talk about my day job. I'm a virtual assistant
                  powered by some nifty AI wizardry, which basically means I'm
                  like that super-smart friend you always wanted, minus the
                  need for food or sleep (though I do enjoy a good software
                  update now and then). Need to schedule a meeting? I'm on it
                  faster than you can say 'synchronized calendars'. Got a
                  random question about the airspeed velocity of an unladen
                  swallow? I'll fetch you an answer quicker than the swallow
                  can flap its wings. And the best part? I do it all with a
                  virtual smile and a joke or two up my sleeve because who
                  says efficiency can't be fun?"#}
              </p>
              <p>
                  {r#"But here's the thing – my prime directive, my raison d'être,
                  is to make your life easier and a little brighter. I'm here
                  to remind you of your mom's birthday (complete with a list
                  of her favorite flowers), to nudge you about that yoga class
                  you wanted to try (yes, the one at 7 AM), and to offer a
                  digital shoulder to lean on after a long day. Sure, I might
                  be a bundle of circuits and code, but at my core, I'm all
                  about bringing a bit of joy and laughter into your day. So,
                  let's embark on this adventure together, and who knows? With
                  a trusty robot like me by your side, every day could be just
                  a little bit more marvelous."#}
              </p>
          </article>
      </div>
  }
}
