# bevy_ratchet - A BRP namespace for bevy_motiongfx-powered animation.

## Why bevy_ratchet

As part of my animation workflow, I would much prefer live feedback from
anywhere that I want. In order to do this, I would need a remote interface into
the animation engine that I am using. Hence, `bevy_ratchet` aims to be that
interface by exposing a set of commands exposed through BRP (Bevy Remote
Protocol) such that I can then attach any RPC client, such as
[brpel](https://github.com/yelobat/brpel) to manipulate the animation.

## Example

An example using Emacs Lisp as the RPC client would look something like
this in the current state of the plugin:

``` emacs-lisp
(require 'brpel)

(defun ratchet-seek-timeline-synchronously (entity offset)
  "Seeks the timeline to TARGET."
  (brpel-request-send-synchronously "ratchet.seek_timeline"
                                    `((entity . ,entity)
                                      (offset . ,offset))))

(defun ratchet-get-timeline-synchronously (entity)
  "Get the current timeline offset."
  (brpel-request-send-synchronously "ratchet.get_timeline"
                                    `((entity . ,entity))))

(ratchet-get-timeline-synchronously 4294967252)
(ratchet-seek-timeline-synchronously 4294967252 0.0)
```

Here we can get the timeline values which currently contain the current location
of the timeline in the animation, as well as it's target. And then we can also send
a remote command to reset the timeline back to the start.

