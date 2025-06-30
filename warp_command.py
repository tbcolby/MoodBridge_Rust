#!/usr/bin/env python3
"""
WARP COMMAND - Development Intelligence System
Analyzes Warp terminal logs to provide development insights
"""

import re
import os
import json
from datetime import datetime, timezone
from collections import defaultdict, Counter
from pathlib import Path

class WarpLogAnalyzer:
    def __init__(self, log_path=None):
        if log_path is None:
            home = os.path.expanduser("~")
            log_path = f"{home}/Library/Logs/warp.log"
        
        self.log_path = log_path
        self.timestamp_pattern = re.compile(r'(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})')
        self.command_patterns = {
            'cargo': re.compile(r'cargo\s+(build|run|test|check|clippy)'),
            'git': re.compile(r'git\s+(add|commit|push|pull|status|diff|log)'),
            'vim': re.compile(r'(vim|nvim)\s+'),
            'code': re.compile(r'code\s+'),
            'ls': re.compile(r'\bls\b'),
            'cd': re.compile(r'\bcd\s+'),
            'grep': re.compile(r'\bgrep\s+'),
            'find': re.compile(r'\bfind\s+'),
            'cat': re.compile(r'\bcat\s+'),
        }

    def analyze_today(self):
        """Analyze today's Warp log activity"""
        print("🚀 WARP COMMAND - Development Intelligence System")
        print("=" * 50)
        print()
        
        if not os.path.exists(self.log_path):
            print(f"❌ Log file not found: {self.log_path}")
            return
        
        today = datetime.now().date()
        commands = []
        
        print(f"🔍 Analyzing Warp logs for {today}...")
        print(f"📁 Log file: {self.log_path}")
        
        try:
            with open(self.log_path, 'r', encoding='utf-8', errors='ignore') as f:
                for line_num, line in enumerate(f, 1):
                    if line_num % 10000 == 0:
                        print(f"📊 Processed {line_num:,} lines...")
                    
                    # Extract timestamp
                    timestamp_match = self.timestamp_pattern.search(line)
                    if not timestamp_match:
                        continue
                    
                    try:
                        timestamp_str = timestamp_match.group(1)
                        timestamp = datetime.fromisoformat(timestamp_str + "+00:00")
                        
                        # Only analyze today's logs
                        if timestamp.date() != today:
                            continue
                        
                        # Look for command patterns
                        command_info = self.extract_command_info(line, timestamp)
                        if command_info:
                            commands.append(command_info)
                            
                    except ValueError:
                        continue
                        
        except Exception as e:
            print(f"❌ Error reading log file: {e}")
            return
        
        print(f"✅ Analysis complete!")
        print(f"📈 Found {len(commands)} command executions today")
        print()
        
        # Analyze patterns
        self.analyze_patterns(commands)

    def extract_command_info(self, line, timestamp):
        """Extract command information from a log line"""
        for cmd_name, pattern in self.command_patterns.items():
            if pattern.search(line):
                return {
                    'timestamp': timestamp,
                    'command': cmd_name,
                    'line': line.strip()
                }
        return None

    def analyze_patterns(self, commands):
        """Analyze development patterns from commands"""
        if not commands:
            print("❌ No commands found in today's logs")
            return
        
        print("🎯 WARP COMMAND Analysis Results")
        print("=" * 32)
        
        # Command frequency analysis
        command_freq = Counter(cmd['command'] for cmd in commands)
        
        print("📊 Command Frequency:")
        for cmd, count in command_freq.most_common():
            print(f"   {cmd}: {count}x")
        print()
        
        # Time analysis
        if commands:
            first_cmd = min(commands, key=lambda x: x['timestamp'])
            last_cmd = max(commands, key=lambda x: x['timestamp'])
            duration = last_cmd['timestamp'] - first_cmd['timestamp']
            
            print("⏰ Active Time Analysis:")
            print(f"   First command: {first_cmd['timestamp'].strftime('%H:%M:%S')}")
            print(f"   Last command: {last_cmd['timestamp'].strftime('%H:%M:%S')}")
            print(f"   Total span: {duration.total_seconds() // 3600:.0f} hours {(duration.total_seconds() % 3600) // 60:.0f} minutes")
            print()
        
        # Development insights
        print("💡 Development Insights:")
        
        git_count = command_freq.get('git', 0)
        cargo_count = command_freq.get('cargo', 0)
        vim_count = command_freq.get('vim', 0) + command_freq.get('code', 0)
        exploration_count = command_freq.get('ls', 0) + command_freq.get('find', 0) + command_freq.get('grep', 0)
        
        insights = []
        
        if git_count > 5:
            insights.append("🔧 High git activity - active version control usage")
        
        if cargo_count > 3:
            insights.append("🦀 Multiple cargo builds - intensive Rust development")
        
        if vim_count > 0:
            insights.append(f"📝 Code editing sessions detected ({vim_count} instances)")
        
        if exploration_count > 10:
            insights.append("🔍 High exploration activity - discovering project structure")
        
        # Activity level assessment
        total_commands = len(commands)
        if total_commands > 100:
            insights.append("🔥 High activity day - significant development work!")
        elif total_commands > 50:
            insights.append("💪 Good productive session")
        elif total_commands > 20:
            insights.append("👍 Moderate development activity")
        else:
            insights.append("🌱 Light development activity")
        
        for insight in insights:
            print(f"   {insight}")
        
        print(f"   🎯 Total commands executed: {total_commands}")
        print()
        
        # Today's development summary
        print("📋 Today's Development Summary:")
        
        # Calculate productivity score (simple algorithm)
        productivity_score = min(10.0, (total_commands / 10) + (git_count * 0.5) + (cargo_count * 0.8))
        
        print(f"   📊 Productivity Score: {productivity_score:.1f}/10")
        
        if cargo_count > 0:
            print(f"   🛠️  Build Activity: {cargo_count} Rust builds")
        
        if git_count > 0:
            print(f"   📝 Version Control: {git_count} git operations")
        
        if exploration_count > 0:
            print(f"   🔍 Code Exploration: {exploration_count} navigation commands")
        
        print()
        print("🚀 WARP COMMAND Report Complete!")
        print("💌 Next steps:")
        print("   • Set up automated daily email reports")
        print("   • Integrate with project management tools")
        print("   • Add machine learning for personalized insights")

def main():
    analyzer = WarpLogAnalyzer()
    analyzer.analyze_today()

if __name__ == "__main__":
    main()
