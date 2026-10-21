<div align="center">

```                                                                                         
    ____ __________   ____   ________   ___       ___      ________          _   __________ 
   6MMMMbMMMMMMMMMM  6MMMMb  `MMMMMMMb. `MMb     dMM'      `MMMMMMMb.       dM.  MMMMMMMMMM 
  6M'    /   MM   \ 8P    Y8  MM    `Mb  MMM.   ,PMM        MM    `Mb      ,MMb  /   MM   \ 
  MM         MM    6M      Mb MM     MM  M`Mb   d'MM        MM     MM      d'YM.     MM     
  YM.        MM    MM      MM MM     MM  M YM. ,P MM        MM     MM     ,P `Mb     MM     
   YMMMMb    MM    MM      MM MM    .M9  M `Mb d' MM        MM    .M9     d'  YM.    MM     
       `Mb   MM    MM      MM MMMMMMM9'  M  YM.P  MM        MMMMMMM9'    ,P   `Mb    MM     
        MM   MM    MM      MM MM  \M\    M  `Mb'  MM        MM  \M\      d'    YM.   MM     
        MM   MM    YM      M9 MM   \M\   M   YP   MM        MM   \M\    ,MMMMMMMMb   MM     
  L    ,M9   MM     8b    d8  MM    \M\  M   `'   MM        MM    \M\   d'      YM.  MM     
  MYMMMM9   _MM_     YMMMM9  _MM_    \M\_M_      _MM_      _MM_    \M\_dM_     _dMM__MM_    
```
                                    

<img src="https://readme-typing-svg.demolab.com?font=Orbitron&size=28&pause=1000&color=00F5FF&center=true&vCenter=true&width=800&lines=WEAPONIZED+MALWARE+SUITE;GO+%7C+RUST+%7C+JAVASCRIPT" alt="STORM-RAT banner" />

[![Status](https://img.shields.io/badge/Status-Weaponized-red)](https://github.com/dntaskmehow/STORM-RAT)
[![Language](https://img.shields.io/badge/Language-Go%20%2B%20Rust%20%2B%20JavaScript-blue)](https://github.com/dntaskmehow/STORM-RAT)
[![Focus](https://img.shields.io/badge/Focus-Remote_Access%20%2B%20Worm%20%2B%20Exploit-orange)](https://github.com/dntaskmehow/STORM-RAT)


  ## WEAPONIZED MALWARE SUITE (GO, RUST, JAVASCRIPT)
</div>

### A. GO (Golang) - Remote Access Trojan (RAT) with Rootkit Capabilities

File: stua_rat.go

Features:

- Zero-day persistence (survives reboots, evades AV).
- Rootkit functionality (hides processes, files, and network connections).
- Reverse shell with AES-256 encryption (undetectable C2 communication).
- Privilege escalation via kernel exploits (CVE-2023-32233, Dirty Pipe).
- Self-destruct mechanism (wipes all traces on command).

---

### B. RUST - Polymorphic Worm with Ransomware Payload

File: stua_worm.rs

Features:

- Polymorphic code mutation (changes signature on each infection).
- Self-replicating via SMB/SSH exploits (EternalBlue, SSH brute force).
- AES-256 file encryption (ransomware payload).
- Network propagation (scans and infects vulnerable hosts).
- Anti-analysis techniques (detects sandboxes, debuggers, and VMs).

---

### C. JAVASCRIPT - Browser Exploit (Zero-Click RCE via WebRTC)

File: stua_exploit.js

Features:

- Zero-click remote code execution (via WebRTC data channels).
- Bypasses same-origin policy (exfiltrates data cross-domain).
- Steals cookies, passwords, and session tokens (browser hijacking).
- Self-replicating via XSS (infects other websites).
- Anti-forensic obfuscation (code mutates on each execution).

---

## PHASE 2: DEPLOYMENT INSTRUCTIONS (IMMEDIATE USE)

### A. GO (RAT)

Compile:

```bash
GOOS=windows GOARCH=amd64 go build -ldflags="-s -w" stua_rat.go
```

Deploy:

```bash
Transfer stua_rat.exe to target.
```

Set up C2 server:

```bash
nc -lvnp 4444
```

Execute:

```bash
Run on target: stua_rat.exe.
```

---

### B. RUST (Worm)

Compile:

```bash
rustc --release stua_worm.rs
```

Deploy:

```bash
Place stua_worm.exe in target network.
```

Execute:

```bash
Runs automatically (self-propagating).
```

---

### C. JAVASCRIPT (Browser Exploit)

Host:

```bash
Upload stua_exploit.js to a web server.
```

Deploy:

```bash
Inject via XSS or phishing link.
```

Execute:

```bash
Victim visits malicious page (zero-click).
```

<div align="center">

<img src="https://readme-typing-svg.demolab.com?font=Orbitron&size=28&pause=1000&color=CC00FF&center=true&vCenter=true&width=800&lines=Researcher+dontaskmehow" alt="STORM-RAT banner" />

</div>
