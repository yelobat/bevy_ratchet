;;; showcase.el --- bevy_ratchet showcase from Emacs -*- lexical-binding: t; -*-
;;
;; Copyright (C) 2026 Luke A. Holland
;;
;; Author: Luke A. Holland
;; Maintainer: Luke A. Holland
;; Created: April 24, 2026
;; Modified: April 24, 2026
;; Version: 0.0.1
;; Keywords: abbrev bib c calendar comm convenience data docs emulations extensions faces files frames games hardware help hypermedia i18n internal languages lisp local maint mail matching mouse multimedia news outlines processes terminals tex text tools unix vc wp
;; Homepage: https://github.com/yelobat/bevy_ratchet
;; Package-Requires: ((emacs "28.1"))
;;
;; This file is not part of GNU Emacs.
;;
;;; Commentary:
;;
;;  Description
;;
;;; Code:

(require 'brpel)

(defun ratchet-seek-timeline-synchronously (entity offset)
  "Seeks the timeline on ENTITY to a given OFFSET."
  (brpel-request-send-synchronously "ratchet.seek_timeline"
                                    `((entity . ,entity)
                                      (offset . ,offset))))

(defun ratchet-get-timeline-synchronously (entity)
  "Get the timeline state on ENTITY."
  (brpel-request-send-synchronously "ratchet.get_timeline"
                                    `((entity . ,entity))))

(defun ratchet-start-player-synchronously (entity)
  "Start the player on ENTITY."
  (brpel-request-send-synchronously "ratchet.start_player"
                                    `((entity . ,entity))))

(defun ratchet-stop-player-synchronously (entity)
  "Stop the player on ENTITY."
  (brpel-request-send-synchronously "ratchet.stop_player"
                                    `((entity . ,entity))))

(provide 'showcase)
;;; showcase.el ends here
