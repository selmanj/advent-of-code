(ns day1 (:require [clojure.java.io :as io]
                   [clojure.string :as str]))

;; input is our input as a string
(def input (-> (io/resource "day1/input.txt")
               slurp))

(def freqs (->> input
                str/split-lines
                (map #(Integer/parseInt %))))

(defn answer1 [] 
  (reduce + 0 freqs))

(defn answer2 []
  (loop [xs (cycle freqs)
         visited #{}
         freq 0]
    (if (contains? visited freq)
      freq
      (recur (rest xs) 
             (conj visited freq)
             (+ freq (first xs))))))