# Discord QA Scraper

## Description
This project aims to retrieve and process Questions and Answers from specific Discord channel messages and threads. It consists of various components developed in Rust and Python, designed to handle chat messages and both active and archived threads for Discord Forum Channels.

Using Serenity and Python Discord libraries.

## Features
- **Discord Chat Messages and Threads Getter (Rust)**: Retrieves messages and threads from specified Discord channel.
- **Chat Processor and Thread Processor**: Separately processes the retrieved chat messages and threads with Open AI API for upserting a Vector DB as Question Answer Pair.
- **Archived and Active Thread Getter/Scraper (Python)**: Gets both active and archived threads from Discord Forum Channel.
- **Thread Processors (Active and Archived)**: Separate processing modules for active and archived threads with Open AI API. Extracts Question and Answer Pairs for each thread.

## Installation

### Prerequisites
- Rust (version 1.26.0 or higher)
- Python (version 3.9 or higher)


### Setup
1. Clone the repository: 
